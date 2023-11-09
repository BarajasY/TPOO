/* @refresh reload */
import { render } from "solid-js/web";
import "./styles/styles.css";
import { Route, Router, Routes } from "@solidjs/router";
import Attendance from "./components/attendance/Attendance";
import Statistics from "./components/statistics/Statistics";
import SideBar from "./components/sidebar/SideBar";
import { onMount } from "solid-js";
import { SectionsEnum, setCurrentDay, setCurrentSection } from "./sharedSignals";
import DBConfig from "./components/DBConfig/DBConfig";
import Config from "./components/DBConfig/Config";
import Events from "./components/events/Events";

onMount(() => {
  const temp = new Date();
  setCurrentDay(new Date(temp.toDateString()))
  setCurrentSection(SectionsEnum.Attendance)
})

render(() =>
  <Router>
    <SideBar />
    <Routes>
      <Route path="/" element={<DBConfig />} />
      <Route path="/statistics" element={<Statistics />} />
      <Route path="/attendance" element={<Attendance />} />
      <Route path="/config" element={<Config />} />
      <Route path="/events" element={<Events />} />
    </Routes>
  </Router>,

  document.getElementById("root") as HTMLElement
);
