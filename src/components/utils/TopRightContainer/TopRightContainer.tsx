import { ParentComponent } from "solid-js"
import "./TopRightContainer.scss"

const TopRightContainer: ParentComponent = (props) => {

  return (
    <div class="container">
      {props.children}
    </div>
  )
}

export default TopRightContainer
