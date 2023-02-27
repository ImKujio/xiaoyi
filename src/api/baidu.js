import md5 from "../utils/md5.js";
import {Body, getClient} from "@tauri-apps/api/http";
const zhPat = new RegExp("[\u4E00-\u9FA5]+");

export default function (query) {
    const appid = '20201014000589277';
    const key = 'OoVc0AO250OLX85blAQB';
    const salt = (new Date).getTime().toString();
    const isZh = zhPat.test(query)
    const sign = md5(appid + query + salt + key);
    return new Promise((resolve, reject) => {
        getClient().then(client => {
            client.post("http://api.fanyi.baidu.com/api/trans/vip/translate",
                Body.form({
                    q: query,
                    appid: appid,
                    salt: salt,
                    from: isZh ? "zh" : "en",
                    to: isZh ? "en" : "zh",
                    sign: sign
                })
            ).then(r => resolve(r.data)).catch(r => reject(r))
        })
    })

}