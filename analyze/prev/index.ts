import { file, serve } from "bun";
import index from "./index.html";
serve({
    routes: {
        "/*": index,
        "/data1": file(`${import.meta.dir}/../data/union.json`),
        "/data2": file(`${import.meta.dir}/../data/option.json`),
        "/data3": file(`${import.meta.dir}/../data/map.json`),
    }
})