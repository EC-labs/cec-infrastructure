import { useContext } from "react";
import { AuthContext } from "./AuthContext";
import { Admin } from "./Admin";
import { Student } from "./Student";

export function Home() {
    const { user: { email, role } } = useContext(AuthContext);

    return (<>
        {role === "admin" ? <Admin/> : <Student/>}
    </>)
}
