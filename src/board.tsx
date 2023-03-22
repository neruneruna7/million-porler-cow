import Card from './tranp_card';
import { useState } from 'react'
import './board.css';
import { invoke } from "@tauri-apps/api/tauri";


interface Response {
    score: number,
    total_num_of_atempt: number,
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
    const [use_cards, setCards] = useState<Array<number>>([]);
    const [input_text, setInputEl] = useState("");
    const [ave_score, setAveScore] = useState(0.0);
    const [max_score, setMaxScore] = useState(0);

    const [response, setResponse] = useState<Response>({
        score: 0,
        total_num_of_atempt: 0,
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

    async function porker() {
        if (use_cards.length < 5) {
            alert("5枚以上のカードを選択してください");
            return;
        }
        const board = await invoke<Response>("million_porker", {});
        setResponse(board);
        setAveScore(response.score / response.total_num_of_atempt);

        if (ave_score > max_score) {
            setMaxScore(ave_score);
        }
    }

    function clicked(id: number) {
        setActive(active.map((act, index) => (index === id ? !act : act)));
        if (use_cards[id] === id) {
            setCards(
                use_cards.filter((card, index) => (card !== id))
            )
        } else {
            setCards([...use_cards, id]);
        }
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
                <input value={input_text} onChange={(e) => setInputEl(e.target.value)} type="text" placeholder="試行回数:最大値100万" />
            </h3>

            <div className="ctr">
                <button className='bu' onClick={porker}>POSTする</button>

                <button className='bu' onClick={reset_card_state}>リセット</button>
                <button className='bu' onClick={() => console.log(active, use_cards)}></button>
            </div>
            <div className='score'>
                <h2>平均スコア！:{ave_score}</h2>
                <h2>最大スコア！ :{max_score}</h2>
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