import { Routes, Route } from "react-router";
import { Login } from './Login.tsx';
import { Home } from './Home.tsx';
import { AuthContext } from './AuthContext.tsx';
import { useEffect, useState } from "react";
import { useNavigate } from "react-router";

function App() {
    const [user, setUser] = useState({email: "", role: ""});
    const [authenticated, setAuthenticated] = useState(false);
    const value = {user, setUser};
    const navigate = useNavigate();
    
    useEffect(() => {
        fetch("/api/users")  
            .then((res) => {
                if (!res.ok) {
                    throw new Error(`Request status not OK: ${res.status}`);
                }
                return res.json();
            })
            .then((body) => {
                setAuthenticated(true);
                setUser({email: body["email"], role: body["role"]})
            })
            .catch((_) => {
                setAuthenticated(true);
                navigate("/login");
            })

    }, [authenticated])

    return (
        <AuthContext.Provider value={value}>
        {authenticated && 
            <Routes>
                <Route path='/' element={<Home/>}/> 
                <Route path='*' element={<Login/>}/> 
            </Routes>
        }
        </AuthContext.Provider>
    )
}

export default App
