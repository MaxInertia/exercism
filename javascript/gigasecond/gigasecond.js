//
// This is only a SKELETON file for the 'Gigasecond' exercise. It's been provided as a
// convenience to get you started writing code faster.
//

export const gigasecond = (t) => {
  const d = t.getTime()
  const p = 10**12
  console.log(d, p, d+p)
  return new Date(d + p)
};
