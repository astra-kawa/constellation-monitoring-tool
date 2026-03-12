include("constellation_model.jl")
using .ConstellationModel
using HTTP
using JSON3
using OrdinaryDiffEq

const HOST = "127.0.0.1"
const PORT = 4001

function parse_float(value, name::AbstractString)
    value isa Real || throw(ArgumentError("$name must be numeric"))
    return Float64(value)
end

function parse_initial_states(payload::Dict{String,Any})
    raw = payload["constellation"]
    raw isa AbstractVector || throw(ArgumentError("constellation must be an array"))
    !isempty(raw) || throw(ArgumentError("constellation must not be empty"))

    states = NTuple{6,Float64}[]
    ids = String[]

    for (index, sat) in pairs(raw)
        sat isa AbstractDict || throw(ArgumentError("constellation[$index] must be an object"))
        push!(ids, String(get(sat, "id", "sat-$(index)")))
        push!(
            states,
            (
                parse_float(get(sat, "pos_x", nothing), "constellation[$index].initial_state.pos_x"),
                parse_float(get(sat, "pos_y", nothing), "constellation[$index].initial_state.pos_y"),
                parse_float(get(sat, "pos_z", nothing), "constellation[$index].initial_state.pos_z"),
                parse_float(get(sat, "vel_x", nothing), "constellation[$index].initial_state.vel_x"),
                parse_float(get(sat, "vel_y", nothing), "constellation[$index].initial_state.vel_y"),
                parse_float(get(sat, "vel_z", nothing), "constellation[$index].initial_state.vel_z"),
            )
        )
    end

    return ids, states
end

function grid_including_end(start::Real, duration::Real, step::Real)
    n = floor(Int, duration/step) + 1
    v = collect(range(start=start, step=step, length=n))
    
    if v[end] != duration
        push!(v, float(duration))
    end
    
    v[end] = float(duration)
    
    return v
end

function build_response(duration::Float64, step::Float64, ids, states)
    spec = ConstellationSpec(length(states); mu = 398600)
    model, symbols = build_constellation_system(spec)
    
    prob = build_constellation_problem(model, symbols, states, (0.0, duration))
    sol = solve(prob, Tsit5())
    
    ts = grid_including_end(0.0, duration, step)
    sample = sol(ts)

    satellites = Vector{Any}(undef, spec.nsat)
    for i in 1:spec.nsat
        pos_x_vector = sample[getproperty(model, symbols.pos_x_name[i])]
        pos_y_vector = sample[getproperty(model, symbols.pos_y_name[i])]
        pos_z_vector = sample[getproperty(model, symbols.pos_z_name[i])]

        trajectory = Vector{Any}(undef, length(ts))
        for k in eachindex(ts)
            trajectory[k] = Dict(
                "t" => ts[k],
                "x" => pos_x_vector[k],
                "y" => pos_y_vector[k],
                "z" => pos_z_vector[k],
            )
        end

        satellites[i] = Dict(
            "id" => ids[i],
            "trajectory" => trajectory,
        )
    end

    return Dict(
        "satellites" => satellites,
        "metadata" => Dict(
            "duration_seconds" => duration,
            "step_seconds" => step,
        ),
    )
end

function propagate_handler(req::HTTP.Request)
    payload = isempty(req.body) ? Dict{String,Any}() : JSON3.read(req.body, Dict{String,Any})
    
    duration = parse_float(payload["duration_seconds"], "duration_seconds")
    step = parse_float(payload["step_seconds"], "step_seconds")
    
    duration > 0.0 || throw(ArgumentError("duration_seconds must be > 0"))
    step > 0.0 || throw(ArgumentError("step_seconds must be > 0"))
    step <= duration || throw(ArgumentError("step_seconds must be <= duration_seconds"))

    ids, states = parse_initial_states(payload)
    response_body = build_response(duration, step, ids, states)
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
