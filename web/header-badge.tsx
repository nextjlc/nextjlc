/* web/header-badge.tsx */

export const HeaderBadge = ({
  software,
  count,
}: {
  software: string;
  count: number;
}) => {
  if (!software || count === 0) return null;

  const styles = {
    Altium: {
      badge: "bg-red-600/80 text-white",
      count: "bg-red-800 text-white",
    },
    KiCad: {
      badge: "bg-blue-600/80 text-white",
      count: "bg-blue-800 text-white",
    },
    EasyEDA: {
      badge: "bg-sky-500/80 text-white",
      count: "bg-sky-700 text-white",
    },
    None: {
      badge: "bg-[#4a4540] text-[#a8a090]",
      count: "bg-[#3a3530] text-[#a8a090]",
    },
  };

  const selectedStyle = styles[software as keyof typeof styles] || styles.None;

  return (
    <span
      className={`relative ml-2 px-2 py-0.5 rounded-full text-xs font-medium ${selectedStyle.badge}`}
    >
      {software}
      <span
        className={`absolute -top-1.5 -right-1.5 flex items-center justify-center
                  w-4 h-4 rounded-full text-[10px] font-bold shadow
                  ${selectedStyle.count}`}
      >
        {count}
      </span>
    </span>
  );
};
