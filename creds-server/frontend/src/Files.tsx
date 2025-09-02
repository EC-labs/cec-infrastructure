import { useContext, useEffect, useState } from "react";
import { AuthContext } from "./AuthContext";
import DownloadIcon from '@mui/icons-material/Download';

import "./files.css";

export function Files() {
    const { user } = useContext(AuthContext);
    const [files, setFiles] = useState({});
    const [download, setDownload] = useState<undefined | string>(undefined);

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

    useEffect(() => {
        if (!download) {
            return; 
        }

        fetch(`/api/download/${download}`)
            .then((res) => {
                if (!res.ok) {
                    throw new Error(`Request status not OK: ${res.status}`);
                }

                let disposition = res.headers.get('Content-Disposition');
                let filename = 'download';
                if (disposition && disposition.includes('filename=')) {
                    filename = disposition.split('filename=')[1].replace(/"/g, '');
                }

                return res.blob().then(blob => ({ blob, filename }));
            })
            .then(({ blob, filename }) => {
                const url = window.URL.createObjectURL(blob);
                const a = document.createElement('a');
                a.href = url;
                a.download = filename;
                a.style.display = 'none';
                document.body.appendChild(a);
                a.click();
                document.body.removeChild(a);
                window.URL.revokeObjectURL(url);
            })
            .catch(console.error);
        setDownload(undefined);
    }, [download])

    function onDownload(key) {
        return (e) => {
            setDownload(`${key}.zip`);
        }
    }

    let fileElements = [];
    for (const [key, entries] of Object.entries(files)) {
        fileElements.push(
            <div className="directory" key={key}>
                <p>{key}</p>
                <div onClick={onDownload(key)}>
                    <DownloadIcon/>
                </div>
            </div>
        );
        for (const entry of entries) {
            fileElements.push(<p key={key + "-" + entry} style={{marginLeft: "25px"}}>{entry}</p>);
        }
        fileElements.push(<p key={key + "-"}/>)
    }

    return (
        <div className="files-page">
            <div className="files-container">
                {fileElements}
            </div>
        </div>
    );
}
