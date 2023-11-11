import { A } from "@solidjs/router";
import "./Return.scss";

//Esencialmente un botón reutilizable que te regresa una pagina hacia atrás.
export const Return = () => {
  return (
    <div class="return-button">
      <A href="..">
        <button>Regresar</button>
      </A>
    </div>
  )
}

export default Return
