import type { Satellite } from "./types";

export async function getConstellationData(): Promise<Satellite[]> {
  let constellationData: Satellite[] = [];

  try {
    const constellationResponse = await fetch(
      "http://127.0.0.1:3000/api/constellation",
    );
    if (!constellationResponse.ok) {
      throw new Error("backend unavailable");
    }

    const response = await constellationResponse.json();
    constellationData = response.satellites;
  } catch (error) {
    console.log(`Error: ${error}`);
  }

  return constellationData;
}
