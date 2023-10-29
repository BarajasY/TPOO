import { A } from "@solidjs/router";
import { IoStatsChartSharp } from 'solid-icons/io'
import { FaRegularCalendarCheck } from 'solid-icons/fa'
import { CurrentSection, SectionsEnum, setCurrentSection } from "../../sharedSignals";
import "./SideBar.scss"

const SideBar = () => {

  return (
    <div
      class="appSideBar">

      <A href="/attendance"
        classList={{ 'active': CurrentSection() === SectionsEnum.Attendance }}
        onClick={() => setCurrentSection(SectionsEnum.Attendance)}>
        <FaRegularCalendarCheck class="icon"/>
      </A>
      <A href="/statistics"
        classList={{ 'active': CurrentSection() === SectionsEnum.Statistics }}
        onClick={() => setCurrentSection(SectionsEnum.Statistics)}>
        <IoStatsChartSharp class="icon"/>
      </A>
    </div>
  );
};

export default SideBar;
