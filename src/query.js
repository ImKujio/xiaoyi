import {getClient} from "@tauri-apps/api/http";
import MD5 from "js-md5";
import CryptoJS from "crypto-js";

export default {
    target: [
        {name: "中", value: "zh"},
        {name: "英", value: "en"},
    ],
    auto: function (query) {
        let chinese = query.match(/[\u4e00-\u9fa5]/g);  // 匹配汉字
        if (!chinese) return 0
        return chinese.length <= (query.length / 2) ? 0 : 1;
    },
    query: async function (text, target) {
        const appKey = "46981bd46f84b2c0"
        const key = "LcCDUNqLFqkaNIau7yidYSiMSUGnHszD"
        const salt = (new Date).getTime();
        const curTime = Math.round(new Date().getTime() / 1000);
        const from = "auto"

        function truncate(q) {
            var len = q.length;
            if (len <= 20) {
                return q;
            }
            return q.substring(0, 10) + len + q.substring(len - 10, len);
        }

        const str1 = appKey + truncate(text) + salt + curTime + key;
        const sign = CryptoJS.SHA256(str1).toString(CryptoJS.enc.Hex);
        const client = await getClient()
        const result = await client.get(
            `https://openapi.youdao.com/api?`
            + `q=${encodeURIComponent(text)}`
            + `&appKey=${appKey}`
            + `&salt=${salt}`
            + `&from=${from}`
            + `&to=${target}`
            + `&sign=${sign}`
            + `&signType=v3`
            + `&curtime=${curTime}`
        )
        if (result.data && result.data.translation) {
            return result.data.translation
        }
        throw new Error(JSON.stringify(result.data))
    }

    // query: async function (text, target) {
    //     const appid = '20230331001623007';
    //     const key = 'xfLERtGHNxxSOczfPcNm';
    //     const salt = (new Date).getTime().toString();
    //     const sign = MD5(appid + text + salt + key);
    //     const query = encodeURIComponent(text)
    //     const client = await getClient()
    //     const url = `http://api.fanyi.baidu.com/api/trans/vip/translate?q=${query}&appid=${appid}&salt=${salt}&from=auto&to=${target}&sign=${sign}`
    //     const result = await client.get(url)
    //     if (result.data.trans_result)
    //         return result.data.trans_result.map(d => d.dst)
    //     throw new Error(JSON.stringify(result.data))
    // }
}
