


export function stringToArray(content: string): number[] {
    return [...new TextEncoder().encode(content)];
}
