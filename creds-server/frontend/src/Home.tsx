import { useContext } from "react";
import { AuthContext } from "./AuthContext";
import { Admin } from "./Admin";
import { Files } from "./Files";

export function Home() {
    const { user: { role } } = useContext(AuthContext);

    return (<>
        {role === "admin" ? <Admin/> : <Files/>}
    </>)
}
