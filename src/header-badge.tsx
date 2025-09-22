/* src/header-badge.tsx */

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
      badge: "bg-red-500/80 text-white",
      count: "bg-red-700 text-white",
    },
    KiCad: {
      badge: "bg-blue-500/80 text-white",
      count: "bg-blue-700 text-white",
    },
    EasyEDA: {
      badge: "bg-sky-400/80 text-white",
      count: "bg-sky-600 text-white",
    },
    None: {
      badge: "bg-gray-500/80 text-white",
      count: "bg-gray-700 text-white",
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
