import { bench } from "./base";


const smi: [number] = [0];
bench("smi", () => {}, () => {
    smi[0] = (smi[0] + 1) & 0xFFFF;
});

const unsmi: [number] = [0];
bench("unsmi", () => {}, () => {
    unsmi[0]++;
});
