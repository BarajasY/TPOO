import { invoke } from "@tauri-apps/api";
import "./Apply.scss";
import { createSignal } from "solid-js";
//@ts-ignore
import {createMotion} from "@motionone/solid";

const Revert = () => {
  let ref: HTMLDivElement | undefined;

  const [Success, setSuccess] = createSignal<boolean>(false)

  const applyMigrations = () => {
    invoke('revert_migrations').then(res => {
      setSuccess(res as boolean);
      //animation stuffs
      createMotion(ref, {
        animate: { x: '-100%'},
        transition: { duration: 1.5 }
      });

      setTimeout(() => {
        createMotion(ref, {
          animate: { x: '0%'},
          transition: { duration: 1.5 }
        })
      }, 1500);
    })
  }

  return (
    <div class="apply-button">
      <button onClick={() => applyMigrations()}>
        Revertir Migraciones
      </button>
      <div
        class="shadow"
        ref={ref}
        >{Success() ? "Migraciones revertidas" : "Error al revertir Migraciones"}</div >
    </div>
  )
}

export default Revert
