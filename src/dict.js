import SQLite from "tauri-plugin-sqlite-api";
import {resourceDir} from "@tauri-apps/api/path";


const dict = {
    ins: null,
    async query(world) {
        if (this.ins == null) {
            const resDir = await resourceDir()
            this.ins = await SQLite.open(resDir+"\\dict.db")
        }
        let rst = await this.ins.select("SELECT dst FROM en WHERE word LIKE ?", [world]);
        if (rst.length === 0) return null;
        return JSON.parse(rst[0].dst);
    }
}

export default dict