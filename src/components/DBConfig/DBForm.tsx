import { Component } from "solid-js";
import "./DBForm.scss"
import { setTempDatabaseCredentials } from "../../sharedSignals";

const DBForm: Component = () => {
  return (
    <div>
      <div class="input-text-container">
        <p class="label">Puerto</p>
        <input type="number" class="input" placeholder="Ej. 1402" oninput={(e) => setTempDatabaseCredentials(dc => ({...dc!, db_port:Number(e.target.value)}))} />
      </div>
      <div class="input-text-container">
        <p class="label">Usuario</p>
        <input type="text" class="input" placeholder="Ej. postgres" oninput={(e) => setTempDatabaseCredentials(dc => ({...dc!, db_user: e.target.value}))} />
      </div>
      <div class="input-text-container">
        <p class="label">Contraseña</p>
        <input type="password" class="input" placeholder="Ej. yvkbevif" oninput={(e) => setTempDatabaseCredentials(dc => ({...dc!, db_pass: e.target.value}))} />
      </div>
      <div class="input-text-container">
        <p class="label">Host</p>
        <input type="text" class="input" placeholder="Ej. localhost" oninput={(e) => setTempDatabaseCredentials(dc => ({...dc!, db_host: e.target.value}))} />
      </div>
      <div class="input-text-container">
        <p class="label">Base de Datos</p>
        <input type="text" class="input" placeholder="Ej. tpoo" oninput={(e) => setTempDatabaseCredentials(dc => ({...dc!, db_name: e.target.value}))} />
      </div>
    </div>
  );
};

export default DBForm;
