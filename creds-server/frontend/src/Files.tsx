import { useContext, useEffect, useState } from "react";
import { AuthContext } from "./AuthContext";

import "./files.css";

export function Files() {
    const { user } = useContext(AuthContext);
    const [files, setFiles] = useState([]);

    useEffect(() => {
        fetch("/api/files")
            .then((res) => {
                if (!res.ok) {
                    throw new Error(`Request status not OK: ${res.status}`);
                }
                return res.json();
            })
            .then((body) => {
                setFiles(body);
            })
            .catch((e) => {
                console.log(e)
            })
    }, [])

    return (
        <div className="files-page">
            <div className="files-container">
                {files.map((file) => <p key={file}>{file}</p>)}
            </div>
        </div>
    );
}
