import Navbar from "react-bootstrap/Navbar";
import { useTranslation } from "react-i18next";

export default function PublicViewHeader() {
    const { i18n } = useTranslation()

    return (<Navbar bg="light" expand="lg">
        <button onClick={() => i18n.changeLanguage("en")} role="img" aria-label="British Flag">🇬🇧</button>
        <button onClick={() => i18n.changeLanguage("sv")} role="img" aria-label="Swedish Flag">🇸🇪</button>
    </Navbar>);
}