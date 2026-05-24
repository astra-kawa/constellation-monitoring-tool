include("constellation_model.jl")
using .ConstellationModel
using HTTP, JSON3, OrdinaryDiffEq, Dates

const HOST = "127.0.0.1"
const PORT = 4001

const DT_FORMAT = dateformat"yyyy-mm-ddTHH:MM:SSZ";
epochs = Dict{String,DateTime}()

function parse_float(value, name::AbstractString)
    value isa Real || throw(ArgumentError("$name must be numeric"))
    return Float64(value)
end

function parse_initial_states(payload::Dict{String,Any})
    raw = payload["constellation"]["satellites"]
    raw isa AbstractVector || @error "constellation.satellites must be an array"
    !isempty(raw) || @error "constellation.satellites must not be empty"

    states = NTuple{6,Float64}[]
    ids = String[]

    for (index, satellite) in enumerate(raw)
        @info "Parsing satellite $index" satellite
        satellite isa AbstractDict || @error "constellation.satellites[$index] must be an object"

        push!(ids, String(get(satellite, "id", "sat-$(index)")))

        push!(
            states,
            (
                parse_float(get(satellite["data"]["initial_state"], "pos_x", nothing), "constellation.satellites[$index].initial_state.pos_x"),
                parse_float(get(satellite["data"]["initial_state"], "pos_y", nothing), "constellation.satellites[$index].initial_state.pos_y"),
                parse_float(get(satellite["data"]["initial_state"], "pos_z", nothing), "constellation.satellites[$index].initial_state.pos_z"),
                parse_float(get(satellite["data"]["initial_state"], "vel_x", nothing), "constellation.satellites[$index].initial_state.vel_x"),
                parse_float(get(satellite["data"]["initial_state"], "vel_y", nothing), "constellation.satellites[$index].initial_state.vel_y"),
                parse_float(get(satellite["data"]["initial_state"], "vel_z", nothing), "constellation.satellites[$index].initial_state.vel_z"),
            )
        )

        push!(epochs, satellite["id"] => DateTime(satellite["data"]["initial_state"]["epoch"], DT_FORMAT))
    end

    return ids, states
end

function grid_including_end(start::Real, duration::Real, step::Real)
    n = floor(Int, duration / step) + 1
    v = collect(range(start=start, step=step, length=n))

    if v[end] != duration
        push!(v, float(duration))
    end

    v[end] = float(duration)

    return v
end

function build_response(duration::Float64, step::Float64, ids, states)
    spec = ConstellationSpec(length(states); mu=398600)
    model, symbols = build_constellation_system(spec)

    prob = build_constellation_problem(model, symbols, states, (0.0, duration))
    sol = solve(prob, Tsit5())

    ts = grid_including_end(0.0, duration, step)
    sample = sol(ts)

    satellites = Vector{Any}(undef, spec.nsat)
    for i in 1:spec.nsat
        cartesian_ephemeris = Vector{Any}(undef, length(ts))
        keplerian_ephemeris = Vector{Any}(undef, length(ts))

        pos_x_vector = sample[getproperty(model, symbols.pos_x_name[i])]
        pos_y_vector = sample[getproperty(model, symbols.pos_y_name[i])]
        pos_z_vector = sample[getproperty(model, symbols.pos_z_name[i])]
        vel_x_vector = sample[getproperty(model, symbols.vel_x_name[i])]
        vel_y_vector = sample[getproperty(model, symbols.vel_y_name[i])]
        vel_z_vector = sample[getproperty(model, symbols.vel_z_name[i])]

        for (k, time) in enumerate(ts)
            datetime = epochs[ids[i]] + Second(time)

            cartesian_ephemeris[k] = Dict(
                "datetime" => Dates.format(datetime, DT_FORMAT),
                "pos_x" => pos_x_vector[k],
                "pos_y" => pos_y_vector[k],
                "pos_z" => pos_z_vector[k],
                "vel_x" => vel_x_vector[k],
                "vel_y" => vel_y_vector[k],
                "vel_z" => vel_z_vector[k],
                "reference_frame" => "J2000",
                "source" => "Predicted"
            )

            keplerian_ephemeris[k] = Dict(
                "datetime" => Dates.format(datetime, DT_FORMAT),
                "sma_km" => 0.0,
                "eccentricity" => 0.0,
                "inclination_deg" => 0.0,
                "raan_deg" => 0.0,
                "arg_periapsis_deg" => 0.0,
                "true_anomaly_deg" => 0.0,
                "reference_frame" => "J2000",
                "source" => "Predicted"
            )
        end

        satellites[i] = Dict(
            "id" => ids[i],
            "cartesian_ephemeris" => cartesian_ephemeris,
            "keplerian_ephemeris" => keplerian_ephemeris,
        )
    end

    return Dict(
        "satellites" => satellites,
    )
end

function propagate_handler(req::HTTP.Request)
    payload = isempty(req.body) ? Dict{String,Any}() : JSON3.read(req.body, Dict{String,Any})

    duration = parse_float(payload["duration_seconds"], "duration_seconds")
    step = parse_float(payload["step_seconds"], "step_seconds")

    duration > 0.0 || throw(ArgumentError("duration_seconds must be > 0"))
    step > 0.0 || throw(ArgumentError("step_seconds must be > 0"))
    step <= duration || throw(ArgumentError("step_seconds must be <= duration_seconds"))
    @info "Parsed duration and step"

    ids, states = parse_initial_states(payload)
    @info "Parsed constellation data"

    response_body = build_response(duration, step, ids, states)
    @info "Generated response body"
    # @info response_body

    return HTTP.Response(
        200,
        ["Content-Type" => "application/json"],
        JSON3.write(response_body),
    )
end

function route(req::HTTP.Request)
    if req.method == "GET" && req.target == "/health"
        println("GET health check")
        return HTTP.Response(200, ["Content-Type" => "application/json"], JSON3.write(Dict("status" => "Ok")))
    end

    if req.method == "POST" && req.target == "/propagate"
        try
            return propagate_handler(req)
        catch err
            @error err
            return HTTP.Response(
                400,
                ["Content-Type" => "application/json"],
                JSON3.write(Dict("error" => sprint(showerror, err))),
            )
        end
    end

    return HTTP.Response(404, ["Content-Type" => "application/json"], JSON3.write(Dict("error" => "not found")))
end

println("Orbit engine running on http://$(HOST):$(PORT)")

HTTP.serve(route, HOST, PORT)
