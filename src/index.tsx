/* @refresh reload */
import { onMount } from "solid-js";
import { render } from "solid-js/web";
import { Router } from "@solidjs/router";
import SideBar from "./components/sidebar/SideBar";
import RouterComponent from "./RouterComponent";
import { SectionsEnum, setCurrentDay, setCurrentSection } from "./sharedSignals";
import "./styles/styles.css";

onMount(() => {
  const temp = new Date();
  setCurrentDay(new Date(temp.toDateString()))
  setCurrentSection(SectionsEnum.Attendance)
})

render(() =>
  <Router>
    <SideBar />
    <RouterComponent />
  </Router>,

  document.getElementById("root") as HTMLElement
);
