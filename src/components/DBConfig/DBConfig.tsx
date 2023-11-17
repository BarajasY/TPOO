import { BaseDirectory, readTextFile, writeFile, exists, createDir } from "@tauri-apps/api/fs";
import { Component, Show, createSignal, onMount } from "solid-js";
import "./DBConfig.scss";
import { DatabaseCredentials, TempDatabaseCredentials, setDatabaseCredentials, setTempDatabaseCredentials } from "../../sharedSignals";
import DBForm from "./DBForm";
import { invoke } from "@tauri-apps/api";
import { useNavigate } from "@solidjs/router";

const DBConfig: Component = () => {
  const navigate = useNavigate();

  const [ErrorMessage, setErrorMessage] = createSignal<boolean>(false);

  onMount(async () => {
    if(!await exists("excel", {dir: BaseDirectory.AppLocalData})) {
      await createDir('excel', {dir: BaseDirectory.AppLocalData});
    }
    let verify = await exists("config.json", {dir: BaseDirectory.AppLocalData});
    let data;
    if(verify) {
      data = await readTextFile("config.json", { dir: BaseDirectory.AppLocalData });
    } else {
      console.log("Creando archivo config.json de default en carpeta AppData/tpoo")
      await writeFile("config.json", JSON.stringify(TempDatabaseCredentials()), {dir: BaseDirectory.AppLocalData});
      data = await readTextFile("config.json", { dir: BaseDirectory.AppLocalData });
    }
    setDatabaseCredentials(JSON.parse(data));
    if(DatabaseCredentials()?.db_pass !== "placeholder") {
      await runWithCredentials();
    }
  })

  const submitCredentials = async () => {
    await writeFile("config.json", JSON.stringify(TempDatabaseCredentials()), {dir: BaseDirectory.AppLocalData})
    invoke("make_database", {credentials: TempDatabaseCredentials()})
    .then(async res => {
      if(res) {
        invoke("run_migrations")
        .then(res => {
          if(res) {
            invoke('change_sala_name', {name: DatabaseCredentials()?.db_table})
            .then(res => {
              if(res) {
                console.log("Todas las migraciones han sido aplicadas!")
                setDatabaseCredentials(TempDatabaseCredentials())
                navigate("/attendance")
              }
            })
          }
        })
      } else {
        setTempDatabaseCredentials({
          db_name: "",
          db_host: "",
          db_port: 0,
          db_pass: "placeholder",
          db_user: "",
          db_table: ""
        })
        await writeFile("config.json", JSON.stringify(TempDatabaseCredentials()), {dir: BaseDirectory.AppLocalData})
        setErrorMessage(true);
        }
      })
  }

  const runWithCredentials = async () => {
    invoke("make_database", {credentials: DatabaseCredentials()})
      .then(res => {
        if(res) {
          invoke("run_migrations")
            .then(res => {
              if(res) {
                invoke('change_sala_name', {name: DatabaseCredentials()?.db_table})
                .then(res => {
                  if(res) {
                    console.log("Todas las migraciones han sido aplicadas!")
                    navigate("/attendance")
                  }
                })
              }
            })
        }
      })
  }

  return (
    <Show when={DatabaseCredentials()?.db_pass === "placeholder"}>
      <div class="db-config-initial">
        <div class="header">
          <p>Primero configuremos tu base de datos!</p>
          <p>Porfavor, ingrese sus credenciales de su base de datos PostgreSQL</p>
          <Show when={ErrorMessage()}>
            <p class="error">Hubo un error con tus credenciales, asegúrate de que sean las correctas.</p>
          </Show>
        </div>
        <div class="form">
          <DBForm />
        </div>
        <button class="submit-form" onclick={() => submitCredentials()}>Enviar</button>
      </div>
    </Show>
  );
};

export default DBConfig;
