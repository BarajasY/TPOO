import { Component } from 'solid-js';
import './TopText.scss'

type Props = {
  text: string | undefined
};

const TopText:Component<Props> = (props: Props) => {
  return (
    <div class="top-text">
      <p class='text'>{props.text}</p>
    </div>
  )
}

export default TopText
