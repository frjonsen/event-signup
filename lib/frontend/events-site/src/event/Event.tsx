import { JSX, useEffect, useState } from "react";
import { useParams } from "react-router";
import { Translation } from "../languages/common";
import PublicViewHeader from "../common/PublicViewHeader";
import { useTranslation} from "react-i18next";
import { i18n } from "i18next";
import * as axios from "axios";
import { handleRequestError } from "../error";


interface Contact {
    organizer: string | undefined;
    email: string | undefined;
    emailVisible: boolean;
    phone: string | undefined;
}

interface Location {
    name: string;
    link: string;
}

interface Event {
    id: string;
    title: Translation,
    signupEndDate: Date;
    eventDate: Date;
    location: Location;
    contact: Contact;
    description: Translation;
    limit: number | undefined;
    image: string | null;
    visible: boolean;
}

function renderEventImages(event: Event): JSX.Element{
    if (event.image === null) {
        return <></>;
    }

    let url = `/static/events/${event.id}/${event.image}.avif`;
    if (window.location.hostname === "localhost") {
        url = `https://events.jonsen.se${url}`;
    }
    return <img className="eventImage" src={url} alt="Picture from event location" />;

}

function renderEvent(event: Event, translator: i18n): JSX.Element {
    // Clone translator so we don't pollute the global resources with temporary resources
    const eventTranslator = translator.cloneInstance();

    Object.keys(event.title).forEach((key) => {
        eventTranslator.addResource(key, "translation", "title", event.title[key]);
        eventTranslator.addResource(key, "translation", "description", event.description[key]);
    });

    console.log(eventTranslator.getResource("en", "translation", "location"))

    return (
        <div>
            {renderEventImages(event)}
            <h1>{eventTranslator.t("title")}</h1>
            <p>{eventTranslator.t("description")}</p>
            <p>📅 {event.eventDate.toLocaleString("sv-SE",  { year: "numeric" })}</p>
            <p>📍 <a href={event.location.link}>{event.location.name}</a></p>
            <p>✉ {event.contact.email}</p>
            <p>📞 {event.contact.phone}</p>
            <p>👥 {event.limit}</p>
        </div>
    )
}

export default function Event() {
    const { id } = useParams();
    const [errorCode, setErrorCode] = useState<string | null>(null);
    const [event, setEvent] = useState<Event | null>(null);

    const {t, i18n } = useTranslation();

    useEffect(() => {
        axios.default.get<Event>(`/api/public/event/${id}`)
            .then((response) => {
                setEvent(response.data);
            })
            .catch((error) => {
                handleRequestError(error, setErrorCode)
        });
    }, [id])

    let inner: JSX.Element;
    if (event !== null) {
        inner = renderEvent(event, i18n);
    } else if (errorCode !== null) {
        inner = <div>{t(`backend.${errorCode}`)}</div>;
    } else {
        inner = <div>{t("loading")}...</div>;
    }

    return (
        <div>
            <PublicViewHeader />
            {inner}
        </div>
    );
}