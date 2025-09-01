import { Routes, Route } from "react-router";
import { Login } from './Login.tsx';
import { Home } from './Home.tsx';
import { AuthContext } from './AuthContext.tsx';
import { useState } from "react";

function App() {
    const [user, setUser] = useState("");
    const value = {user, setUser};
    return (
        <AuthContext.Provider value={value}>
        <Routes>
            <Route path='/' element={<Home/>}/> 
            <Route path='*' element={<Login/>}/> 
        </Routes>
        </AuthContext.Provider>
    )
}

export default App
