import {getClient} from "@tauri-apps/api/http";
import MD5 from "js-md5";

const zhPat = new RegExp("[\u4E00-\u9FA5]+");

export default {
    target:[
        {name: "中", value: "zh"},
        {name: "英", value: "en"},
    ],
    query:async function(text,target){
        const appid = '20201014000589277';
        const key = 'OoVc0AO250OLX85blAQB';
        const salt = (new Date).getTime().toString();
        const sign = MD5(appid + text + salt + key);
        const query = encodeURIComponent(text)
        const client = await getClient()
        const url = `http://api.fanyi.baidu.com/api/trans/vip/translate?q=${query}&appid=${appid}&salt=${salt}&from=auto&to=${target}&sign=${sign}`
        const result = await client.get(url)
        if (result.data.trans_result)
            return result.data.trans_result.map(d => d.dst)
        throw new Error(JSON.stringify(result.data))
    }
}
