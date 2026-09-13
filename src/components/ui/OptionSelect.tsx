import { useState, useRef, useEffect } from "react";

export interface SelectOption {
  value: string;
  label: string;
  description?: string;
  icon?: string;
}

interface OptionSelectProps {
  options: SelectOption[];
  value: string;
  onChange: (value: string) => void;
  placeholder?: string;
  label?: string;
  compact?: boolean;
}

export function OptionSelect({
  options,
  value,
  onChange,
  placeholder = "Select...",
  label,
  compact = false,
}: OptionSelectProps) {
  const [open, setOpen] = useState(false);
  const ref = useRef<HTMLDivElement>(null);
  const selected = options.find((o) => o.value === value);

  useEffect(() => {
    if (!open) return;
    const handler = (e: MouseEvent) => {
      if (ref.current && !ref.current.contains(e.target as Node)) {
        setOpen(false);
      }
    };
    document.addEventListener("mousedown", handler);
    return () => document.removeEventListener("mousedown", handler);
  }, [open]);

  return (
    <div className="option-select-wrapper" ref={ref}>
      {label && <label className="form-label">{label}</label>}
      <button
        className={`option-select-trigger ${compact ? "compact" : ""} ${open ? "open" : ""}`}
        onClick={() => setOpen(!open)}
        type="button"
      >
        <span className="option-select-value">
          {selected?.icon && <span className="option-select-icon">{selected.icon}</span>}
          {selected ? selected.label : <span className="option-select-placeholder">{placeholder}</span>}
        </span>
        <span className={`option-select-chevron ${open ? "rotated" : ""}`}>▾</span>
      </button>
      {open && (
        <div className="option-select-dropdown">
          {options.map((opt) => (
            <button
              key={opt.value}
              className={`option-select-item ${opt.value === value ? "selected" : ""}`}
              onClick={() => {
                onChange(opt.value);
                setOpen(false);
              }}
              type="button"
            >
              {opt.icon && <span className="option-select-item-icon">{opt.icon}</span>}
              <div className="option-select-item-content">
                <div className="option-select-item-label">{opt.label}</div>
                {opt.description && (
                  <div className="option-select-item-desc">{opt.description}</div>
                )}
              </div>
              {opt.value === value && <span className="option-select-check">✓</span>}
            </button>
          ))}
        </div>
      )}
    </div>
  );
}

interface OptionGridProps {
  options: SelectOption[];
  value: string;
  onChange: (value: string) => void;
  label?: string;
}

export function OptionGrid({ options, value, onChange, label }: OptionGridProps) {
  return (
    <div className="option-grid-wrapper">
      {label && <label className="form-label">{label}</label>}
      <div className="option-grid">
        {options.map((opt) => (
          <button
            key={opt.value}
            className={`option-grid-item ${opt.value === value ? "selected" : ""}`}
            onClick={() => onChange(opt.value)}
            type="button"
          >
            {opt.icon && <span className="option-grid-icon">{opt.icon}</span>}
            <span className="option-grid-label">{opt.label}</span>
            {opt.value === value && <span className="option-grid-check">✓</span>}
          </button>
        ))}
      </div>
    </div>
  );
}
