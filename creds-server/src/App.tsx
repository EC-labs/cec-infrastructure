import { Routes, Route } from "react-router";
import { Login } from './Login.tsx';
import {Home} from './Home.tsx';

function App() {
  return (
    <Routes>
        <Route path='/' element={<Home/>}/> 
        <Route path='*' element={<Login/>}/> 
    </Routes>
  )
}

export default App
