%% Thermistor Wheatstone Bridge Calculator.
% Tool to design a Wheatstone bridge for a Pt100, Pt500 or Pt1000
% thermistor. Interactively helps in choosing R = R1 = R2 and R3.
clear all, close all, clc

% Chosen values:
% V = 3.3V
% Tmin = -50 C
% Tmax = +50 C
% Roffset = 1000 Ohms
% Ratio = 4

V = input("Supply voltage (V_in): ");
Tmin = input("Input minimum temperature of range (Celsius): ");
Tmax = input("Input maximum temperature of range (Celsius): ");
Roffset = input("Current-limiter resistance (Ohms): ");
Rmin = Roffset + temp2ohms(Tmin);
Rmax = Roffset + temp2ohms(Tmax);



TABLEHEADER = sprintf("Component \t %3.1f deg. C \t %3.1f deg. C\n", ...
    Tmin, Tmax);
TABLEROW1   = sprintf("Pt100  \t\t %3.1f Ohms \t %3.1f Ohms\n", ...
    Rmin(1), Rmax(1));
TABLEROW2   = sprintf("Pt500  \t\t %3.1f Ohms \t %3.1f Ohms\n", ...
    Rmin(2), Rmax(2));
TABLEROW3   = sprintf("Pt1000 \t\t %3.1f Ohms \t %3.1f Ohms\n", ...
    Rmin(3), Rmax(3));

disp(" ");
disp("TABLE: Temperature-Ohm range of the different Pt-resistors");
disp(TABLEHEADER);
disp(TABLEROW1);
disp(TABLEROW2);
disp(TABLEROW3);

R3 = Rmin;
R = sqrt((R3 .* Rmax.^2 - Rmax .* R3.^2) ./ (Rmax - R3)); % R1 and R2.

%% Current analysis - MINIMUM current.
figure(1)
R_ratio_range = 1:0.1:10;
R_pt100  = R(1) * R_ratio_range;
R_pt500  = R(2) * R_ratio_range;
R_pt1000 = R(3) * R_ratio_range;

Imin_pt100  = V ./ (Rmax(1) + R_pt100);
Imin_pt500  = V ./ (Rmax(2) + R_pt500);
Imin_pt1000 = V ./ (Rmax(3) + R_pt1000);

Irec = [1e-3, 0.7e-3, 0.1e-3];
Ilim = [7e-3, 3e-3, 1e-3];

plot(R_ratio_range, Imin_pt100, 'b');
hold on
plot(R_ratio_range, Imin_pt500, 'r');
plot(R_ratio_range, Imin_pt1000, 'g');
plot([R_ratio_range(1), R_ratio_range(end)], [Irec(1) Irec(1)], '--b');
plot([R_ratio_range(1), R_ratio_range(end)], [Irec(2) Irec(2)], '--r');
plot([R_ratio_range(1), R_ratio_range(end)], [Irec(3) Irec(3)], '--g');
plot([R_ratio_range(1), R_ratio_range(end)], [Ilim(1) Ilim(1)], '-.b');
plot([R_ratio_range(1), R_ratio_range(end)], [Ilim(2) Ilim(2)], '-.r');
plot([R_ratio_range(1), R_ratio_range(end)], [Ilim(3) Ilim(3)], '-.g');
legend("Pt100", "Pt500", "Pt1000", ...
    "Recommended: Pt100", "Pt500", "Pt1000", ...
    "Limit: Pt100", "Pt500", "Pt1000");
xlabel("R / R_{max V_T}")
ylabel("I_{min} (A)")

%% Current analysis - MAXIMUM current.
figure(2)
R_ratio_range = 1:0.1:10;
R_pt100  = R(1) * R_ratio_range;
R_pt500  = R(2) * R_ratio_range;
R_pt1000 = R(3) * R_ratio_range;

Imax_pt100  = V ./ (Rmin(1) + R_pt100);
Imax_pt500  = V ./ (Rmin(2) + R_pt500);
Imax_pt1000 = V ./ (Rmin(3) + R_pt1000);

Irec = [1e-3, 0.7e-3, 0.1e-3];
Ilim = [7e-3, 3e-3, 1e-3];

plot(R_ratio_range, Imax_pt100, 'b');
hold on
plot(R_ratio_range, Imax_pt500, 'r');
plot(R_ratio_range, Imax_pt1000, 'g');
plot([R_ratio_range(1), R_ratio_range(end)], [Irec(1) Irec(1)], '--b');
plot([R_ratio_range(1), R_ratio_range(end)], [Irec(2) Irec(2)], '--r');
plot([R_ratio_range(1), R_ratio_range(end)], [Irec(3) Irec(3)], '--g');
plot([R_ratio_range(1), R_ratio_range(end)], [Ilim(1) Ilim(1)], '-.b');
plot([R_ratio_range(1), R_ratio_range(end)], [Ilim(2) Ilim(2)], '-.r');
plot([R_ratio_range(1), R_ratio_range(end)], [Ilim(3) Ilim(3)], '-.g');
legend("Pt100", "Pt500", "Pt1000", ...
    "Recommended: Pt100", "Pt500", "Pt1000", ...
    "Limit: Pt100", "Pt500", "Pt1000");
xlabel("R / R_{max V_T}")
ylabel("I_{max} (A)")

%% VTmax analysis. (Output range)
figure(3)
VTmax_pt100 = ...
    V * (Rmax(1) ./ (R_pt100 + Rmax(1)) - R3(1) ./ (R_pt100 + R3(1)));
VTmax_pt500 = ...
    V * (Rmax(2) ./ (R_pt500 + Rmax(2)) - R3(2) ./ (R_pt500 + R3(2)));
VTmax_pt1000 = ...
    V * (Rmax(3) ./ (R_pt1000 + Rmax(3)) - R3(3) ./ (R_pt1000 + R3(3)));


plot(R_ratio_range, VTmax_pt100, 'b');
hold on
plot(R_ratio_range, VTmax_pt500, 'r');
plot(R_ratio_range, VTmax_pt1000, 'g');
legend("Pt100", "Pt500", "Pt1000")
xlabel("R / R_{max V_T}")
ylabel("Voltage range V_T (V)");

%% Look at graphs and set R = R1 = R2.
disp("R_maxVT is value of R = R1 = R2 yielding max. output range VTmax.");
ratio = input("Based on the graphs, select ratio R/R_maxVT: ");
R = ratio * R;

VTmax = V * (Rmax ./ (R + Rmax) - R3 ./ (R + R3)); % Maximum output, range.
Imax = V ./ (Rmin + R); % Maximum current for range.
Itot = V ./ parallelR(Rmin + R, R3 + R);

Imax = Imax * 1000; % mA.
Itot = Itot * 1000; % mA.
VTmax = VTmax * 1000; % mV.


TABLECAPTION = sprintf(...
    "\nTABLE: Current consumptions and VT ranges for ratio = %3.1f.",...
    ratio);
TABLEHEADER = sprintf(...
    "Component \t R3 \t\t\t R \t\t\t\t VTrange \t Imax Rx \t Itot\n");
TABLEROW1 = sprintf(...
    "Pt100  \t\t %3.2f Ohms \t %3.2f Ohms \t %3.1f mV \t %3.3f mA \t %3.3f mA\n", ...
    R3(1), R(1), VTmax(1), Imax(1), Itot(1));
TABLEROW2 = sprintf(...
    "Pt500  \t\t %3.2f Ohms \t %3.2f Ohms \t %3.1f mV \t %3.3f mA \t %3.3f mA\n", ...
    R3(2), R(2), VTmax(2), Imax(2), Itot(2));
TABLEROW3 = sprintf(...
    "Pt1000 \t\t %3.2f Ohms \t %3.2f Ohms \t %3.1f mV \t %3.3f mA \t %3.3f mA\n", ...
    R3(3), R(3), VTmax(3), Imax(3), Itot(3));

disp(TABLECAPTION);
disp(TABLEHEADER);
disp(TABLEROW1);
disp(TABLEROW2);
disp(TABLEROW3);

%% Functions
function [R] = temp2ohms(T)
% PT100, PT500, PT1000 components' dependence of resistance upon
% temperature (Celsius).
% INPUT:
%   T - Temperature in Celsius.
% OUTPUT:
%   R - Three resistances in Ohms, format: [PT100, PT500, PT1000].

N = length(T);

R0 = [100, 500, 1000]; % Pt100, Pt500, Pt1000 resistors.

% EN/IEC 60751 curve fit parameters.
A = 3.90802e-3;     % deg^-1
B = -5.775e-7;      % deg^-2
C = -4.2735e-12;    % deg^-4

R = [0, 0, 0];
for i = 1:3
    if T < 0
        R(i) = R0(i) * (1 + A * T + B * T^2 + C * (T - 100) * T^3);
    else
        R(i) = R0(i) * (1 + A * T + B * T^2);
    end
end

end
function [R] = parallelR(R1, R2)
% Parallel resistances equivalent value.
R = (R1 .* R2) ./ (R1 + R2);

end