import { useContext, useState } from "react";
import { jwtDecode } from 'jwt-decode';
import { AuthContext } from './AuthContext';
import { useNavigate } from "react-router";

type JWTPayload = {
    role: string,
    user: string,
};

export function Login() {
    const [input, setInput] = useState("");
    const [placeholder, setPlaceholder] = useState("token");
    const { setUser } = useContext(AuthContext);
    const navigate = useNavigate();

    function handleInputChange(e: React.ChangeEvent<HTMLInputElement>) {
        console.log(e.target.value);
        setInput(e.target.value);
    }

    function onLogin(_e: React.MouseEvent<HTMLElement>) {
        try {
            const payload = jwtDecode<JWTPayload>(input);
            // validate token w/ backend
            // store user as http cookie
            // setUser
            // navigate("/");
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
