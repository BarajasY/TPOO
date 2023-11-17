import { Component } from "solid-js"
import './MakeExcel.scss'
import { invoke } from "@tauri-apps/api"
import { appLocalDataDir } from "@tauri-apps/api/path"
import { createSignal } from "solid-js";
//@ts-ignore
import { createMotion } from "@motionone/solid";

type props = {
  label: string,
  cantidad: number,
  name: string
}

const MakeExcel: Component<props> = (props: props) => {

  let ref: HTMLDivElement | undefined;

  const [Success, setSuccess] = createSignal<boolean>(false)
  console.log(props.name)

  const makeExcel = async () => {
    invoke('make_xlsx', { since: props.cantidad, path: await appLocalDataDir(), name: props.name })
      .then(res => {
        setSuccess(res as boolean);
        //animation stuffs
        createMotion(ref, {
          animate: { x: '-100%' },
          transition: { duration: 1.5 }
        });

        setTimeout(() => {
          createMotion(ref, {
            animate: { x: '0%' },
            transition: { duration: 1.5 }
          })
        }, 1500);
      })
  }

  return (
    <div class="make-excel">
      <button onClick={() => makeExcel()}>
        Hacer excel del {props.label}
      </button>
      <div
        class="shadow"
        ref={ref}
      >{Success() ? "Excel creado" : "Error al crear Excel"}</div >
    </div>
  )
}

export default MakeExcel

