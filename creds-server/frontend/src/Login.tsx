import { useContext, useEffect, useState } from "react";
import { jwtDecode } from 'jwt-decode';
import { AuthContext } from './AuthContext';
import { useNavigate } from "react-router";

type JWTPayload = {
    role: string,
    user: string,
};

export function Login() {
    const [token, setToken] = useState("");
    const [login, setLogin] = useState(false);
    const [placeholder, setPlaceholder] = useState("token");
    const { setUser } = useContext(AuthContext);
    const navigate = useNavigate();

    useEffect(() => {
        if (login) {
            fetch(`/api/authenticate?token=${token}`)
                .then(res => {
                    if (res.status == 200) {
                        const payload = jwtDecode<JWTPayload>(token);
                        setUser({email: payload.user, role: payload.role});
                        navigate("/")
                    } else {
                        setToken("");
                        setPlaceholder("invalid token");
                    }
                });
            setLogin(false)
        }
    }, [login]);

    function handleInputChange(e: React.ChangeEvent<HTMLInputElement>) {
        setToken(e.target.value);
    }

    return (
        <>
            <h1>Login</h1>
            <div>
                <input type="password" value={token} placeholder={placeholder} onChange={handleInputChange}/>
                <button onClick={(_) => setLogin(true)}>login</button>
            </div>
        </>
    );
}
