import { Route, Routes } from "@solidjs/router"
import Config from "./components/DBConfig/Config"
import DBConfig from "./components/DBConfig/DBConfig"
import Attendance from "./components/attendance/Attendance"
import Events from "./components/events/Events"
import Statistics from "./components/statistics/Statistics"
import EventsControls from "./components/events/EventsControls"
import EventsAttendance from "./components/events/eventsAttendance/EventsAttendance"
import EventsInvite from "./components/events/eventsInvite/EventsInvite"

export const RouterComponent = () => {
  return (
    <Routes>
      <Route path="/" element={<DBConfig />} />
      <Route path="/statistics" element={<Statistics />} />
      <Route path="/attendance" element={<Attendance />} />
      <Route path="/config" element={<Config />} />
      <Route path="/events" element={<Events />} />
      <Route path="/events/:id" element={<EventsControls />} />
      <Route path="/events/:id/attendance" element={<EventsAttendance />} />
      <Route path="/events/:id/invite" element={<EventsInvite />} />
    </Routes>
  )
}

export default RouterComponent
