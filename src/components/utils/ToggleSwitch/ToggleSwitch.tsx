import "./ToggleSwitch.scss";

const ToggleSwitch = () => {
  return (
    <div class="toggle-switch">
      <label class="switch">
        <input type="checkbox" />
          <span class="slider round"></span>
      </label>
    </div>
  )
}

export default ToggleSwitch;
