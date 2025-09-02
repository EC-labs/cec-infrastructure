import ReactDOM from "react-dom/client";
import { BrowserRouter } from "react-router";
import App from "./App";

import './main.css';

const root = document.getElementById("root");

ReactDOM.createRoot(root as HTMLElement).render(
    <BrowserRouter>
        <App/>
    </BrowserRouter>,
);
