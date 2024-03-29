import Card from './tranp_card';
import { useState } from 'react'
import './board.css';
import { invoke } from "@tauri-apps/api/tauri";


interface Request {
    num_of_atempt: number,
    use_cards: Array<number>,
}

interface Response {
    score: number,
    total_num_of_atempt: number,
    time_ms: number,
    result: Result,
}

interface Result {
    nopair: number,
    onepair: number,
    twopair: number,
    threepair: number,
    fourpair: number,
    fulhouse: number,
    flush: number,
    strait: number,
    straitflush: number,
    royalflush: number,
}

export default function Board() {
    const [active, setActive] = useState<Array<boolean>>(Array(52).fill(false));
    const [input_text, setInputEl] = useState("");
    const [ave_score, setAveScore] = useState(0.0);
    const [max_score, setMaxScore] = useState(0);



    const [response, setResponse] = useState<Response>({
        score: 0,
        total_num_of_atempt: 0,
        time_ms: 0,
        result: {
            nopair: 0,
            onepair: 0,
            twopair: 0,
            threepair: 0,
            fourpair: 0,
            fulhouse: 0,
            flush: 0,
            strait: 0,
            straitflush: 0,
            royalflush: 0,
        }
    });

    async function porker(active: Array<boolean>) {
        let use_cards: Array<number> = [];
        for (let id = 0; id < 52; id++) {
            if (active[id] === true) {
                use_cards.push(id);
            }
        }
        const num = parseInt(input_text);
        if (use_cards.length < 5) {
            alert("5枚以上のカードを選択してください");
            return;
        } else if (Number.isNaN(num)) {
            alert("有効な数字を入力してください");
            return;
        }

        const request: Request = { num_of_atempt: num, use_cards: use_cards };

        await invoke<Response>("million_porker", { request: request }).then((board) => {
            setResponse(board);

            if (response.score === 0 || response.total_num_of_atempt == 0) {
                setAveScore(0);
            } else {
                setAveScore(response.score / response.total_num_of_atempt);
            }
            if (ave_score > max_score) {
                setMaxScore(ave_score);
            }
        }).catch(e => {
            alert(e);
        })
    }

    function clicked(id: number) {
        setActive(active.map((act, index) => (index === id ? !act : act)));
    }

    function render_card(id: number) {
        return (
            <div onClick={() =>
                clicked(id)
            }>
                <Card id={id} active={active[id]}></Card>
            </div >
        )
    }

    function reset_card_state() {
        setActive(Array(52).fill(false));
        setAveScore(0);
        setMaxScore(0);
    }

    return (
        <div className='board'>
            <h3>
                <input value={input_text} onChange={(e) => setInputEl(e.target.value)} type="text" placeholder="試行回数:最大値1000万" />
            </h3>

            <div className="ctr">
                <button className='bu' onClick={() => porker(active)}>実行する</button>
                <button className='bu' onClick={reset_card_state}>リセット</button>
                <button className='bu' onClick={() => {
                    setActive(Array(52).fill(true))
                    console.log(active)
                }
                }></button>
            </div>
            <div className='score'>
                <h2>平均スコア！:{ave_score}</h2>
                <h2>最大スコア！ :{max_score}</h2>
                <h2>Rust側の処理時間 :{response.time_ms}ms</h2>
            </div>
            <div className='board-row'>

                {render_card(0)}
                {render_card(1)}
                {render_card(2)}
                {render_card(3)}
                {render_card(4)}
                {render_card(5)}
                {render_card(6)}
                {render_card(7)}
                {render_card(8)}
                {render_card(9)}
                {render_card(10)}
                {render_card(11)}
                {render_card(12)}
            </div>
            <div className='board-row'>
                {render_card(13)}
                {render_card(14)}
                {render_card(15)}
                {render_card(16)}
                {render_card(17)}
                {render_card(18)}
                {render_card(19)}
                {render_card(20)}
                {render_card(21)}
                {render_card(22)}
                {render_card(23)}
                {render_card(24)}
                {render_card(25)}
            </div>
            <div className='board-row'>
                {render_card(26)}
                {render_card(27)}
                {render_card(28)}
                {render_card(29)}
                {render_card(30)}
                {render_card(31)}
                {render_card(32)}
                {render_card(33)}
                {render_card(34)}
                {render_card(35)}
                {render_card(36)}
                {render_card(37)}
                {render_card(38)}
            </div>
            <div className='board-row'>
                {render_card(39)}
                {render_card(40)}
                {render_card(41)}
                {render_card(42)}
                {render_card(43)}
                {render_card(44)}
                {render_card(45)}
                {render_card(46)}
                {render_card(47)}
                {render_card(48)}
                {render_card(49)}
                {render_card(50)}
                {render_card(51)}
            </div>

            <div className='response'>
                <h2>出現回数!</h2>
                <ul>
                    <div className='role_row'>
                        <li>ノーペア: {response.result.nopair}</li>
                        <li>ワンペア: {response.result.onepair}</li>
                        <li>ツーペア: {response.result.twopair}</li>
                        <li>スリーカード: {response.result.threepair}</li>
                    </div>
                    <div className='role_row'>
                        <li>フォーカード: {response.result.fourpair}</li>
                        <li>フルハウス: {response.result.fulhouse}</li>
                        <li>フラッシュ: {response.result.flush}</li>
                        <li>ストレート: {response.result.strait}</li>
                    </div>
                    <div className='role_row'>
                        <li>ストレートフラッシュ: {response.result.straitflush}</li>
                        <li>ロイヤルストレートフラッシュ！！: {response.result.royalflush}</li>
                    </div>
                </ul>
            </div>
        </div>

    )
}