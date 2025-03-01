import i18next from "i18next";
import { initReactI18next } from "react-i18next";
import LanguageDetector from 'i18next-browser-languagedetector';

//Import all translation files
import English from "./languages/swedish.json";
import Swedish from "./languages/english.json";

const resources = {
    en: {
        translation: English,
    },
    sv: {
        translation: Swedish,
    },
}

i18next.use(initReactI18next)
.use(LanguageDetector)
.init({
    resources,
    fallbackLng: "en",
});

export default i18next;
