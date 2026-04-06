import { file, serve } from "bun";
import index from "./index.html";
serve({
    routes: {
        "/*": index,
        "/data1": file(`${import.meta.dir}/../data/smi.json`),
        "/data2": file(`${import.meta.dir}/../data/unsmi.json`),
    }
})