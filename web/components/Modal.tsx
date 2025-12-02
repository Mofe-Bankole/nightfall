import React from "react";

export type ModalProps = {
  isOpen: boolean;
  action: string;
  onClose: () => void;
};

export default function Modal({ isOpen, onClose }: ModalProps) {
  if (!isOpen) return null;

  return (
    <div
      className="fixed icanset-0 z-50 flex items-center justify-center"
      style={{ backdropFilter: "blur(6px)" }}
    >
      {/* Overlay to catch clicks outside the modal box */}
      <div
        className="absolute inset-0 bg-black/60"
        onClick={onClose}
        style={{ zIndex: 0 }}
      />
      {/* Modal content */}
      <div className="relative bg-white text-black px-8 py-6 rounded-md shadow-lg z-10 min-w-[260px] flex flex-col items-center">
        <div className="mb-2 text-lg font-semibold">Hello</div>
        <button
          className="mt-2 px-4 py-2 rounded bg-black text-white hover:bg-gray-800"
          onClick={onClose}
        >
          Close
        </button>
      </div>
    </div>
  );
}
