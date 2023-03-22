import { useState } from "react";
import './tranp_card.css'

interface CardState {
    id: number;
    active: boolean;
}

export default function Card(card_state: CardState) {
    // const card_id: number = 0;

    let error_str: string = "";

    if (card_state.id > 51) {
        error_str = "無効な値です"
    }
    const card_suit: number = Math.floor(card_state.id / 13);
    const card_rank = (card_state.id % 13) + 1;

    const img_path: string = `/cards/card_id_ (${card_state.id}).webp`;

    //console.log(img_path);


    const [active, setActive] = useState(false);

    function Toggle() {
        setActive(!active);
    }


    return (
        <div className="Card" >
            <button onClick={Toggle} className={card_state.active ? "clicked card_img" : "unclicked card_img"}>
                <img src={img_path}></img>
            </button>
        </div>
    )
}