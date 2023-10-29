import { BaseDirectory, readTextFile, writeFile } from "@tauri-apps/api/fs";
import { Component, Show, onMount } from "solid-js";
import "./DBConfig.scss";
import { DatabaseCredentials, TempDatabaseCredentials, setDatabaseCredentials } from "../../sharedSignals";
import DBForm from "./DBForm";
import { invoke } from "@tauri-apps/api";
import { useNavigate } from "@solidjs/router";

const DBConfig: Component = () => {
  const navigate = useNavigate();

  onMount(async () => {
    const data = await readTextFile("config.json", { dir: BaseDirectory.AppLocalData });
    setDatabaseCredentials(JSON.parse(data));
    if(DatabaseCredentials()?.db_pass !== "placeholder") {
      await runWithCredentials()
    }
  })

  const submitCredentials = async () => {
    await writeFile("config.json", JSON.stringify(TempDatabaseCredentials()), {dir: BaseDirectory.AppLocalData})
    setDatabaseCredentials(TempDatabaseCredentials())
    invoke("make_database", {credentials: DatabaseCredentials()})
      .then(res => {
        if(res) {
          invoke("run_migrations")
            .then(res => {
              if(res) {
                console.log("All migrations have been applied!")
                navigate("/attendance")
              }
            })
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
                console.log("All migrations have been applied!")
                navigate("/attendance")
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
        </div>
        <div class="form">
          <DBForm />
        </div>
        <button class="submit-form" onclick={() => submitCredentials()}>Submit</button>
      </div>
    </Show>
  );
};

export default DBConfig;
