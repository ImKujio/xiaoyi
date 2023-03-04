import SQLite from "tauri-plugin-sqlite-api";


const dict = {
    ins: null,
    async query(world) {
        if (this.ins == null) {
            this.ins = await SQLite.open(".\\dict.db")
        }
        let rst = await this.ins.select("SELECT dst FROM en WHERE word LIKE ?", [world]);
        if (rst.length === 0) return null;
        return JSON.parse(rst[0].dst);
    }
}

export default dict