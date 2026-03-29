import { writeFileSync } from "fs";

const LOOP_COUNT = 100_0000;
const AVG_LEN = 1000;

export function bench<V>(name: string, value: () => V, fn: (value: V) => unknown) {
    const global_start = performance.now();
    const times: number[] = [];
    for (let i = 0; i < LOOP_COUNT; i++) {
        const v = value();
        const start = performance.now();
        fn(v);
        const end = performance.now();
        times.push(end - start);
    }
    const global_end = performance.now();

    console.log(`${name}:
avg: ${times.reduce((p,c)=>p+c,0) / times.length}
sum: ${global_end - global_start}
`);
    const data: number[] = [];
    for (let i = 0; i < LOOP_COUNT / AVG_LEN; i++) {
        const arr = times.slice(i * AVG_LEN, (i + 1) * AVG_LEN);
        const avg = arr.map(e=>e*1_000000).reduce((p,c)=>p+c, 0) / arr.length;
        data.push(avg);
    }
    writeFileSync(`./data/${name}.json`, JSON.stringify(data));
}
