import { A, useLocation } from "@solidjs/router";
import { IoStatsChartSharp } from 'solid-icons/io'
import { FaRegularCalendarCheck, FaSolidGear } from 'solid-icons/fa'
import { BsCalendarEvent } from 'solid-icons/bs'
import "./SideBar.scss"

const SideBar = () => {
  const location = useLocation()

  return (
    <div
      class="appSideBar">
      <A href="/attendance"
        classList={{ 'active': location.pathname.startsWith('/attendance') }}
      >
        <FaRegularCalendarCheck class="icon"/>
      </A>
      <A href="/statistics"
        classList={{ 'active': location.pathname.startsWith('/statistics') }}
      >
        <IoStatsChartSharp class="icon"/>
      </A>
      <A href="/events"
        classList={{ 'active': location.pathname.startsWith('/events') }}
      >
        <BsCalendarEvent class="icon"/>
      </A>
      <A href="/config"
        classList={{ 'active': location.pathname.startsWith('/config') }}
      >
        <FaSolidGear class="icon"/>
      </A>
    </div>
  );
};

export default SideBar;
