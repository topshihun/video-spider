import React from "react";

interface CloseIconProps {
  onClick: (e: React.MouseEvent) => void;
  size?: number;
  color?: string;
  className?: string;
}

const CloseIcon: React.FC<CloseIconProps> = ({
  onClick,
  size = 20,
  color = "currentColor",
  className = "",
}) => {
  const handleClick = (e: React.MouseEvent) => {
    e.stopPropagation();
    onClick(e);
  };

  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      width={size}
      height={size}
      viewBox="0 0 24 24"
      fill="none"
      stroke={color}
      strokeWidth="2"
      strokeLinecap="round"
      strokeLinejoin="round"
      className={`close-icon ${className}`}
      onClick={handleClick}
      style={{ cursor: "pointer", marginLeft: "4px" }}
    >
      <line x1="18" y1="6" x2="6" y2="18" />
      <line x1="6" y1="6" x2="18" y2="18" />
    </svg>
  );
};

export default CloseIcon;
