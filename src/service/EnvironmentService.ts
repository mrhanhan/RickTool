import { EnvironmentGroup } from "./environment";


export async function getEnvs(): Promise<EnvironmentGroup[]> {
    return Promise.resolve([{name: 'Hello'}, {name: 'Hello'}, {name: 'Hello'}]);
}