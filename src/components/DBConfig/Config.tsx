import { Component, Show, createSignal } from "solid-js";
import DBForm from "./DBForm";
import './DBConfig.scss'
import { invoke } from "@tauri-apps/api";
import { writeFile, BaseDirectory } from "@tauri-apps/api/fs";
import { TempDatabaseCredentials, setDatabaseCredentials, setTempDatabaseCredentials } from "../../sharedSignals";

const Config: Component = () => {

  const [ErrorMessage, setErrorMessage] = createSignal<boolean>(false);
  const [MakeMigrations, setMakeMigrations] = createSignal<boolean>(false);
  const [Success, setSuccess] = createSignal<{type: boolean, message: string}>({type: false, message: ''});

  const Submit = async () => {
    await writeFile("config.json", JSON.stringify(TempDatabaseCredentials()), { dir: BaseDirectory.AppLocalData })
    if (MakeMigrations()) {
      invoke("make_database", { credentials: TempDatabaseCredentials() })
        .then(async res => {
          if (res) {
            invoke("run_migrations")
              .then(res => {
                if (res) {
                  console.log("All migrations have been applied!")
                  setDatabaseCredentials(TempDatabaseCredentials())
                  setSuccess(() => ({
                    type: true,
                    message: 'Las migracioens se han hecho correctamente'
                  }))
                }
              })
          } else {
            setTempDatabaseCredentials({
              db_name: "",
              db_host: "",
              db_port: 0,
              db_pass: "placeholder",
              db_user: ""
            })
            await writeFile("config.json", JSON.stringify(TempDatabaseCredentials()), { dir: BaseDirectory.AppLocalData })
            setErrorMessage(true);
            setSuccess(() => ({
              type: false,
              message: ''
            }))
          }
        })
    } else {
      setSuccess(() => ({
        type: true,
        message: 'Las credenciales han sido actualizadas'
      }))
    }
  }

  return (
    <div class="db-config-initial change">
      <div class="header">
        <p>Si desea cambiar sus credenciales de PostgreSQL, insertelas debajo</p>
        <Show when={ErrorMessage()}>
          <p class="error">Hubo un error con tus credenciales, asegúrate de que sean las correctas.</p>
        </Show>
        <Show when={Success().type}>
          <p class="success">{Success().message}</p>
        </Show>
      </div>
      <div class="form">
        <DBForm />
      </div>
      <div class="buttons">
        <button
          class="make-migrations"
          onClick={() => setMakeMigrations(!MakeMigrations())}
          classList={{ active: MakeMigrations() }}
        >
          Hacer Migraciones
        </button>
        <button class="submit-form" onclick={() => Submit()}>Enviar</button>
      </div>
    </div>
  );
};

export default Config
