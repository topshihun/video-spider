import CloseIcon from "./CloseIcon";
import "../assets/css/CloseableButton.css";

function CloseableButton({
  content,
  style,
  onClick,
  close,
}: {
  content: string;
  style?: React.CSSProperties;
  onClick: () => void;
  close: () => void;
}) {
  return (
    <button onClick={onClick} style={style}>
      {content}
      <CloseIcon className="close-icon" onClick={close} />
    </button>
  );
}

export default CloseableButton;
