import { useParams } from "@solidjs/router";
import "./EventsAttendance.scss";
import Return from "../../utils/return/Return";

const EventsAttendance = () => {
  const params = useParams()

  console.log(params.id);

  return (
    <div class="events-attendance">
        <Return />
    </div>
  )
}

export default EventsAttendance;
