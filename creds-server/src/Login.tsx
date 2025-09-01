import { useState } from "react";
import { jwtDecode } from 'jwt-decode';

export function Login() {
    const [input, setInput] = useState("");
    const [placeholder, setPlaceholder] = useState("token");

    function handleInputChange(e: React.ChangeEvent<HTMLInputElement>) {
        console.log(e.target.value);
        setInput(e.target.value);
    }

    function onLogin(_e: React.MouseEvent<HTMLElement>) {
        try {
            const payload = jwtDecode(input);
        } catch {
            setInput("");
            setPlaceholder("invalid token");
        }
    }

    return (
        <>
            <h1>Login</h1>
            <div>
                <input type="password" value={input} placeholder={placeholder} onChange={handleInputChange}/>
                <button onClick={onLogin}>login</button>
            </div>
        </>
    );
}
