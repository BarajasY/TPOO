/* @refresh reload */
import { render } from "solid-js/web";
import "./styles/styles.css";
import { Route, Router, Routes } from "@solidjs/router";
import Attendance from "./components/attendance/Attendance";
import Statistics from "./components/statistics/Statistics";
import SideBar from "./components/sidebar/SideBar";
import { onMount } from "solid-js";
import { SectionsEnum, setCurrentDay, setCurrentSection } from "./sharedSignals";

onMount(() => {
  const temp = new Date();
  setCurrentDay(new Date(temp.toDateString()))
  setCurrentSection(SectionsEnum.Attendance)
})

render(() =>
  <Router>
    <SideBar />
    <Routes>
      <Route path="/" element={<Attendance />} />
      <Route path="/statistics" element={<Statistics />} />
      <Route path="/attendance" element={<Attendance />} />
    </Routes>
  </Router>,

  document.getElementById("root") as HTMLElement
);
