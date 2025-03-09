import { JSX, useEffect, useState } from "react";
import { useParams } from "react-router";
import { Translation } from "../languages/common";
import PublicViewHeader from "../common/PublicViewHeader";
import { useTranslation} from "react-i18next";
import { i18n } from "i18next";
import * as axios from "axios";
import { handleRequestError } from "../error";
import EventIcon from "@mui/icons-material/Event";
import PlaceIcon from "@mui/icons-material/Place";
import EmailIcon from "@mui/icons-material/Email";
import PhoneAndroid from "@mui/icons-material/PhoneAndroid";
import GroupsIcon from "@mui/icons-material/Groups";
import EmojiPeopleIcon from "@mui/icons-material/EmojiPeople";
import { Grid2, Link, List, ListItem, ListItemIcon, ListItemText, Paper } from "@mui/material";


interface Contact {
    organizer: string | null;
    email: string | null;
    emailVisible: boolean;
    phone: string | null;
}

interface Location {
    name: string;
    link: string;
}

interface Event {
    id: string;
    title: Translation,
    signupEndDate: string;
    eventDate: string;
    location: Location;
    contact: Contact;
    description: Translation;
    limit: number | null;
    image: string | null;
    visible: boolean;
}

function renderEventImage(event: Event): JSX.Element{
    if (event.image === null) {
        return <></>;
    }

    let url = `/static/events/${event.id}/${event.image}.avif`;
    if (window.location.hostname === "localhost") {
        url = `https://events.jonsen.se${url}`;
    }
    return <img src={url} alt="Picture from event location" style={{ maxWidth: "100%" }} />;

}

function renderEvent(event: Event, translator: i18n): JSX.Element {
    // Clone translator so we don't pollute the global resources with temporary resources
    const eventTranslator = translator.cloneInstance();

    Object.keys(event.title).forEach((key) => {
        eventTranslator.addResource(key, "translation", "title", event.title[key]);
        eventTranslator.addResource(key, "translation", "description", event.description[key]);
    });

    console.log(eventTranslator.getResource("en", "translation", "location"))
    // Intl.DateTimeFormat("sv-SE", { year: "numeric" }).format(event.eventDate);

    return (
        <Grid2 direction="column" container spacing={2} marginTop="1rem" display="flex" alignItems="center">
            <Grid2 display="flex" justifyContent="center" size={{ xs: 12, xl: 6, lg: 10, }}>{renderEventImage(event)}</Grid2>
            <Grid2 size={{ xs: 12, md: 6 }}>
                <Paper style={{ paddingLeft: "2rem", paddingRight: "2rem" }} elevation={12}>
                    <h1>{eventTranslator.t("title")}</h1>
                    <p>{eventTranslator.t("description")}</p>
                    <List>
                        <ListItem>
                            <ListItemIcon><EventIcon /></ListItemIcon> 
                            <ListItemText>{new Date(event.eventDate).toLocaleString(translator.resolvedLanguage,  { year: "numeric", month: "long", day: "numeric", hour: "2-digit", minute: "2-digit" })}</ListItemText>
                        </ListItem>
                        <ListItem>
                            <ListItemIcon><PlaceIcon /></ListItemIcon>
                            <ListItemText><Link href={event.location.link}>{event.location.name}</Link></ListItemText>
                        </ListItem>
                        {event.contact.email && <ListItem>
                            <ListItemIcon><EmailIcon /></ListItemIcon>
                            <ListItemText>{event.contact.email}</ListItemText>
                        </ListItem>}
                        {event.contact.organizer && <ListItem>
                            <ListItemIcon><EmojiPeopleIcon /></ListItemIcon>
                            <ListItemText>{event.contact.organizer}</ListItemText>
                        </ListItem>}
                        {event.contact.phone && <ListItem>
                            <ListItemIcon><PhoneAndroid /></ListItemIcon>
                            <ListItemText>{event.contact.phone}</ListItemText>
                        </ListItem>}
                        {event.limit && <ListItem>
                            <ListItemIcon><GroupsIcon /></ListItemIcon>
                            <ListItemText>{eventTranslator.t("participantLimit", {limit: event.limit })}</ListItemText>
                        </ListItem>}
                    </List>
                </Paper>
            </Grid2>
        </Grid2>
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